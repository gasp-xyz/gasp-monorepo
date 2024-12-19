#!/bin/bash
IMAGE_TAG=e82c4b3d9539b81f1cbe48066aed73ba5463fc10 helmfile apply -e prod -lname=sequencer-base -lname=sequencer-base-2 -lname=sequencer-base-3 -lname=sequencer-arb -lname=sequencer-arb-2 -lname=sequencer-arb-3 -lname=sequencer-eth -lname=sequencer-eth-2 -lname=sequencer-eth-3 -e prod --kube-context gke_direct-pixel-353917_europe-west1_gasp-prod
