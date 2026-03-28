# GHCR
variable "ghcr_username" {
  description = "GitHub username for GHCR authentication"
  type = string
  sensitive = true
}

variable "ghcr_auth_token" {
  description = "Personal GitHub access token with read:packages scope"
  type = string
  sensitive = true
}
