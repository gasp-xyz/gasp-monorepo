package main

import (
	"context"
	"log"
	"os"

	"github.com/urfave/cli"

	"github.com/mangata-finance/eigen-layer-monorepo/operator_v1"
)

func main() {
	app := cli.NewApp()
	app.Flags = operator.Flags
	app.Name = "mangata-finalizer-operator"
	app.Usage = "Mangata Finalizer Operator"
	app.Description = "Service that reads block number onchain, finalizes the block, signs, and sends them to the aggregator."

	app.Action = operatorMain
	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Application failed. Message:", err)
	}
}

func operatorMain(ctx *cli.Context) error {

	log.Println("Initializing Operator")
	config, err := operator.NewConfig(ctx)
	if err != nil {
		return err
	}

	operator, err := operator.NewOperatorFromConfig(*config)
	if err != nil {
		return err
	}
	log.Println("initialized operator")

	log.Println("starting operator")
	err = operator.Start(context.Background())
	if err != nil {
		return err
	}
	log.Println("started operator")

	return nil

}
