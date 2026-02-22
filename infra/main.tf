terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "7.20.0"
    }
  }

  backend "gcs" {
    bucket = "starfish-terraform-state"
  }
}

provider "google" {
  project = var.google_cloud_project
  region  = var.google_cloud_region
}
