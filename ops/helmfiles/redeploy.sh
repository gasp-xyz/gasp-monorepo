#!/bin/bash
IMAGE_TAG=78d509568fadbdc665b6a67f43f33d1b86834144 helmfile sync -e prod --kube-context gke_direct-pixel-353917_europe-west1_gasp-prod
