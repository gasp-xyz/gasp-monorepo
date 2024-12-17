#!/bin/bash
IMAGE_TAG=83174d468dca9d6e7fe13d798fb8a373d8fb0f42 helmfile sync -e prod --kube-context gke_direct-pixel-353917_europe-west1_gasp-prod
