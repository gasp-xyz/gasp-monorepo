package actions

import (
	"encoding/json"
	"log"

	"github.com/mangata-finance/eigen-layer-monorepo/operator"
	"github.com/urfave/cli"
)

func PrintOperatorStatus(ctx *cli.Context) error {

	config, err := operator.NewConfig(ctx)
	if err != nil {
		return err
	}
	configJson, err := json.MarshalIndent(config, "", "  ")
	if err != nil {
		log.Fatalf(err.Error())
	}
	log.Println("Config:", string(configJson))

	_, err = operator.NewOperatorFromConfig(*config)
	if err != nil {
		return err
	}

	return nil
}
