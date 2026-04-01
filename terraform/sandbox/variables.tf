# GHCR
variable "ghcr_username" {
  description = "GitHub username for GHCR authentication"
  type        = string
  sensitive   = true
}

variable "ghcr_auth_token" {
  description = "Personal GitHub access token with read:packages scope"
  type        = string
  sensitive   = true
}

# Stripe - Sandbox
variable "stripe_secret_key_sandbox" {
  description = "Stripe secret key for sandbox"
  type        = string
  sensitive   = true
}

variable "stripe_webhook_secret_sandbox" {
  description = "Stripe webhook secret for sandbox"
  type        = string
  sensitive   = true
}

# Starfish - Sandbox
variable "starfish_resend_api_key_sandbox" {
  description = "Resend API key for sandbox"
  type        = string
  sensitive   = true
}

variable "starfish_jwt_secret_sandbox" {
  description = "JWT secret for sandbox"
  type        = string
  sensitive   = true
}
