/// OCSF Category UIDs (8 primary categories)
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcsfCategory {
    SystemActivity = 1,
    Findings = 2,
    IdentityAccessManagement = 3,
    NetworkActivity = 4,
    Discovery = 5,
    ApplicationActivity = 6,
    // Note: Categories 7-8 are reserved/custom
}

impl OcsfCategory {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            Self::SystemActivity => "System Activity",
            Self::Findings => "Findings",
            Self::IdentityAccessManagement => "Identity & Access Management",
            Self::NetworkActivity => "Network Activity",
            Self::Discovery => "Discovery",
            Self::ApplicationActivity => "Application Activity",
        }
    }

    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        match self {
            Self::SystemActivity => "Operating system and device-level events",
            Self::Findings => "Security findings from scanning, detection, and analysis",
            Self::IdentityAccessManagement => {
                "Authentication, authorization, and account management"
            }
            Self::NetworkActivity => "Network connections and traffic",
            Self::Discovery => "Resource and asset discovery",
            Self::ApplicationActivity => "Application-specific events",
        }
    }

    #[allow(dead_code)]
    pub fn uid(&self) -> u32 {
        *self as u32
    }
}
