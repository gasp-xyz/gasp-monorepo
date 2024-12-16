#!/bin/bash
IMAGE_TAG=4aad4a9c917ed04965be1c39f5b5dfc495e27f2d helmfile sync -e prod --kube-context gke_direct-pixel-353917_europe-west1_gasp-prod
