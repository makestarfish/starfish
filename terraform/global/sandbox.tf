data "tfe_project" "sandbox" {
  organization = "starfish-labs"
  name         = "Sandbox"
}

resource "tfe_variable_set" "sandbox" {
  name              = "Sandbox Settings"
  description       = "For variables specific to the sandbox environment"
  organization      = "starfish-labs"
  parent_project_id = data.tfe_project.sandbox.id
}

resource "tfe_variable" "stripe_secret_key_sandbox" {
  key             = "stripe_secret_key_sandbox"
  category        = "terraform"
  description     = "Stripe secret key for sandbox"
  sensitive       = true
  variable_set_id = tfe_variable_set.sandbox.id
}

resource "tfe_variable" "stripe_webhook_secret_sandbox" {
  key             = "stripe_webhook_secret_sandbox"
  category        = "terraform"
  description     = "Stripe webhook secret for sandbox"
  sensitive       = true
  variable_set_id = tfe_variable_set.sandbox.id
}

resource "tfe_variable" "starfish_resend_api_key_sandbox" {
  key             = "starfish_resend_api_key_sandbox"
  category        = "terraform"
  description     = "Resend API key for sandbox"
  sensitive       = true
  variable_set_id = tfe_variable_set.sandbox.id
}

resource "tfe_variable" "starfish_jwt_secret_sandbox" {
  key             = "starfish_jwt_secret_sandbox"
  category        = "terraform"
  description     = "JWT secret for sandbox"
  sensitive       = true
  variable_set_id = tfe_variable_set.sandbox.id
}
