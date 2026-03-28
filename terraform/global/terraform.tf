terraform {
  required_version = ">= 1.14"

  cloud {
    organization = "starfish-labs"

    workspaces {
      name = "global"
    }
  }

  required_providers {
    tfe = {
      source  = "hashicorp/tfe"
      version = "0.75.0"
    }
  }
}