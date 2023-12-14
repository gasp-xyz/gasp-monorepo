package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/urfave/cli"

	"github.com/mangata-finance/eigen-layer-monorepo/aggregator"
)

var (
	// Version is the version of the binary.
	Version   string
	GitCommit string
	GitDate   string
)

func main() {

	app := cli.NewApp()
	app.Flags = aggregator.Flags
	app.Version = fmt.Sprintf("%s-%s-%s", Version, GitCommit, GitDate)
	app.Name = "mangata-finalizer-aggregator"
	app.Usage = "Mangata Finalizer Aggregator"
	app.Description = "Service that sends block number to be finalized by operator nodes."

	app.Action = aggregatorMain
	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln("Application failed.", "Message:", err)
	}
}

func aggregatorMain(ctx *cli.Context) error {

	log.Println("Initializing Aggregator")
	config, err := aggregator.NewConfig(ctx)
	if err != nil {
		return err
	}

	agg, err := aggregator.NewAggregator(config)
	if err != nil {
		return err
	}

	err = agg.Start(context.Background())
	if err != nil {
		return err
	}

	return nil

}
