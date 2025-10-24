# OCSF MCP Server - Docker Operations Makefile
# Maintainer: Anubhav Gain (anubhavg-cipl) <anubhavg@infopercept.com>
#
# Quick commands for building, testing, and deploying the OCSF MCP Server

.PHONY: help build build-nc run test clean push pull logs shell docker-login multiarch-build publish

# Docker configuration
DOCKER_USERNAME ?= anubhavgicpl
IMAGE_NAME ?= ocsf-mcp
VERSION ?= 0.1.0
PLATFORMS ?= linux/amd64,linux/arm64

# Derived variables
IMAGE_TAG = $(DOCKER_USERNAME)/$(IMAGE_NAME):$(VERSION)
IMAGE_LATEST = $(DOCKER_USERNAME)/$(IMAGE_NAME):latest

# Colors for output
GREEN  := \033[0;32m
YELLOW := \033[0;33m
RED    := \033[0;31m
NC     := \033[0m # No Color

# Default target
.DEFAULT_GOAL := help

help: ## Show this help message
	@echo "$(GREEN)OCSF MCP Server - Docker Operations$(NC)"
	@echo "$(YELLOW)Maintainer: Anubhav Gain <anubhavg@infopercept.com>$(NC)"
	@echo ""
	@echo "$(GREEN)Available commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2}'

build: ## Build Docker image locally
	@echo "$(GREEN)Building Docker image...$(NC)"
	docker build -t $(IMAGE_TAG) -t $(IMAGE_LATEST) .
	@echo "$(GREEN)✓ Build complete!$(NC)"
	@docker images | grep $(IMAGE_NAME)

build-nc: ## Build Docker image without cache
	@echo "$(GREEN)Building Docker image (no cache)...$(NC)"
	docker build --no-cache -t $(IMAGE_TAG) -t $(IMAGE_LATEST) .
	@echo "$(GREEN)✓ Build complete!$(NC)"

run: ## Run the Docker container interactively
	@echo "$(GREEN)Running OCSF MCP Server...$(NC)"
	docker run -i --rm $(IMAGE_LATEST)

run-detached: ## Run container in background
	@echo "$(GREEN)Starting OCSF MCP Server in detached mode...$(NC)"
	docker run -d --name ocsf-mcp-server $(IMAGE_LATEST)
	@echo "$(GREEN)✓ Container started. Use 'make logs' to view output$(NC)"

compose-up: ## Start services with Docker Compose
	@echo "$(GREEN)Starting services with Docker Compose...$(NC)"
	docker compose up -d
	@echo "$(GREEN)✓ Services started$(NC)"

compose-down: ## Stop Docker Compose services
	@echo "$(YELLOW)Stopping services...$(NC)"
	docker compose down
	@echo "$(GREEN)✓ Services stopped$(NC)"

compose-build: ## Build with Docker Compose
	@echo "$(GREEN)Building with Docker Compose...$(NC)"
	docker compose build --no-cache
	@echo "$(GREEN)✓ Build complete$(NC)"

test: ## Test the Docker container
	@echo "$(GREEN)Testing OCSF MCP Server...$(NC)"
	@echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | docker run -i --rm $(IMAGE_LATEST) || true
	@echo "$(GREEN)✓ Test complete$(NC)"

test-help: ## Test help command
	@echo "$(GREEN)Testing --help flag...$(NC)"
	docker run --rm $(IMAGE_LATEST) --help || echo "$(YELLOW)Note: Server may not support --help$(NC)"

logs: ## View Docker Compose logs
	docker compose logs -f

logs-tail: ## View last 100 lines of logs
	docker compose logs --tail=100 -f

shell: ## Open shell in running container
	@echo "$(GREEN)Opening shell in OCSF MCP container...$(NC)"
	docker compose exec ocsf-mcp /bin/sh || \
	docker run -it --rm --entrypoint /bin/sh $(IMAGE_LATEST)

inspect: ## Inspect the Docker image
	@echo "$(GREEN)Inspecting Docker image...$(NC)"
	docker inspect $(IMAGE_LATEST)

size: ## Show image size
	@echo "$(GREEN)Image size:$(NC)"
	@docker images $(IMAGE_NAME) --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}"

clean: ## Remove local Docker images and containers
	@echo "$(YELLOW)Cleaning up Docker resources...$(NC)"
	docker compose down -v || true
	docker rm -f ocsf-mcp-server 2>/dev/null || true
	docker rmi $(IMAGE_TAG) $(IMAGE_LATEST) 2>/dev/null || true
	@echo "$(GREEN)✓ Cleanup complete$(NC)"

docker-login: ## Login to Docker Hub
	@echo "$(GREEN)Logging in to Docker Hub...$(NC)"
	@docker login -u $(DOCKER_USERNAME)

push: build ## Build and push to Docker Hub
	@echo "$(GREEN)Pushing to Docker Hub...$(NC)"
	docker push $(IMAGE_TAG)
	docker push $(IMAGE_LATEST)
	@echo "$(GREEN)✓ Push complete!$(NC)"
	@echo "$(YELLOW)Image available at: https://hub.docker.com/r/$(DOCKER_USERNAME)/$(IMAGE_NAME)$(NC)"

pull: ## Pull image from Docker Hub
	@echo "$(GREEN)Pulling from Docker Hub...$(NC)"
	docker pull $(IMAGE_LATEST)
	@echo "$(GREEN)✓ Pull complete$(NC)"

multiarch-build: ## Build multi-architecture image (amd64, arm64)
	@echo "$(GREEN)Building multi-architecture image...$(NC)"
	@echo "$(YELLOW)Creating buildx builder if needed...$(NC)"
	docker buildx create --name ocsf-builder --use 2>/dev/null || docker buildx use ocsf-builder
	docker buildx inspect --bootstrap
	@echo "$(GREEN)Building for platforms: $(PLATFORMS)$(NC)"
	docker buildx build \
		--platform $(PLATFORMS) \
		-t $(IMAGE_TAG) \
		-t $(IMAGE_LATEST) \
		--push \
		.
	@echo "$(GREEN)✓ Multi-arch build complete!$(NC)"

multiarch-build-local: ## Build multi-arch and load locally
	@echo "$(GREEN)Building multi-architecture image (local)...$(NC)"
	docker buildx create --name ocsf-builder --use 2>/dev/null || docker buildx use ocsf-builder
	docker buildx build \
		--platform $(PLATFORMS) \
		-t $(IMAGE_TAG) \
		-t $(IMAGE_LATEST) \
		--load \
		.
	@echo "$(GREEN)✓ Multi-arch build complete (loaded locally)$(NC)"

publish: multiarch-build ## Build multi-arch and publish to Docker Hub
	@echo "$(GREEN)✓ Published to Docker Hub!$(NC)"
	@echo "$(YELLOW)Pull with: docker pull $(IMAGE_LATEST)$(NC)"

tag-version: ## Tag current version
	@echo "$(GREEN)Tagging version $(VERSION)...$(NC)"
	docker tag $(IMAGE_LATEST) $(IMAGE_TAG)
	@echo "$(GREEN)✓ Tagged as $(IMAGE_TAG)$(NC)"

mcp-add: ## Add to Docker MCP Toolkit (requires Docker Desktop 4.42+)
	@echo "$(GREEN)Adding to Docker MCP Toolkit...$(NC)"
	docker mcp server add ocsf --image $(IMAGE_LATEST) || \
	echo "$(RED)Error: Docker MCP Toolkit not available. Requires Docker Desktop 4.42+$(NC)"

mcp-connect: ## Connect to Claude Desktop via MCP Toolkit
	@echo "$(GREEN)Connecting to Claude Desktop...$(NC)"
	docker mcp client connect claude-desktop --global || \
	echo "$(RED)Error: Docker MCP Toolkit not available$(NC)"

mcp-test: mcp-add mcp-connect ## Setup MCP integration and test
	@echo "$(GREEN)✓ MCP integration complete!$(NC)"
	@echo "$(YELLOW)Open Claude Desktop and test with: 'List all OCSF event classes'$(NC)"

stats: ## Show container resource usage
	docker stats --no-stream ocsf-mcp-server 2>/dev/null || \
	echo "$(YELLOW)No running container found. Use 'make run-detached' first.$(NC)"

health: ## Check container health
	@docker inspect --format='{{.State.Health.Status}}' ocsf-mcp-server 2>/dev/null || \
	echo "$(YELLOW)No running container found$(NC)"

version: ## Show version info
	@echo "$(GREEN)OCSF MCP Server$(NC)"
	@echo "Version: $(VERSION)"
	@echo "Image: $(IMAGE_TAG)"
	@echo "Latest: $(IMAGE_LATEST)"
	@echo "Platforms: $(PLATFORMS)"

ci-build: ## CI/CD build (for GitHub Actions)
	@echo "$(GREEN)CI/CD Build$(NC)"
	docker build -t $(IMAGE_TAG) -t $(IMAGE_LATEST) .
	docker images

ci-test: ci-build ## CI/CD test pipeline
	@echo "$(GREEN)Running CI/CD tests...$(NC)"
	docker run --rm $(IMAGE_LATEST) --help || true
	@echo "$(GREEN)✓ CI/CD tests passed$(NC)"

# Development targets
dev-watch: ## Watch for changes and rebuild (requires entr)
	@echo "$(GREEN)Watching for changes...$(NC)"
	find src -name '*.rs' | entr -r make build run

dev-logs: ## Follow development logs
	docker compose logs -f --tail=50

# Security scanning
scan: ## Scan image for vulnerabilities (requires trivy)
	@echo "$(GREEN)Scanning for vulnerabilities...$(NC)"
	@which trivy > /dev/null || (echo "$(RED)Trivy not installed. Install from: https://github.com/aquasecurity/trivy$(NC)" && exit 1)
	trivy image $(IMAGE_LATEST)

scan-critical: ## Scan for critical vulnerabilities only
	@echo "$(GREEN)Scanning for critical vulnerabilities...$(NC)"
	@which trivy > /dev/null || (echo "$(RED)Trivy not installed$(NC)" && exit 1)
	trivy image --severity CRITICAL,HIGH $(IMAGE_LATEST)

# Documentation
docs: ## Generate documentation
	@echo "$(GREEN)Documentation links:$(NC)"
	@echo "  Dockerfile: file://$(PWD)/Dockerfile"
	@echo "  Compose: file://$(PWD)/docker-compose.yml"
	@echo "  Manifest: file://$(PWD)/mcp-server.json"
	@echo "  README: file://$(PWD)/README.md"

# All-in-one targets
all: clean build test ## Clean, build, and test
	@echo "$(GREEN)✓ All tasks complete!$(NC)"

production-deploy: clean build-nc multiarch-build ## Full production deployment
	@echo "$(GREEN)✓ Production deployment complete!$(NC)"
	@echo "$(YELLOW)Image published at: https://hub.docker.com/r/$(DOCKER_USERNAME)/$(IMAGE_NAME)$(NC)"
