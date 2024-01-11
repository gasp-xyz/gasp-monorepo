package main

import (
	"log"
	"os"

	"github.com/mangata-finance/eigen-layer-monorepo/operator"
	"github.com/mangata-finance/eigen-layer-monorepo/operator/plugin/actions"
	"github.com/urfave/cli"
)

func main() {
	app := cli.NewApp()

	app.Flags = operator.Flags
	app.Commands = []cli.Command{
		{
			Name:    "register-operator-with-avs",
			Aliases: []string{"r"},
			Usage:   "registers operator with avs registry",
			Action:  actions.RegisterOperatorWithAvs,
		},
		{
			Name:    "register-operator-with-eigen",
			Usage:   "registers operator with eigenLayer contracts",
			Action:  actions.RegisterOperatorWithEigen,
		},
		{
			Name:    "deregister-operator-with-avs",
			Aliases: []string{"d"},
			Usage:   "deregisters operator with avs registry",
			Action:  actions.DeregisterOperatorWithAvs,
		},
		{
			Name:    "print-operator-status",
			Aliases: []string{"s"},
			Usage:   "prints operator status as viewed from AVS contracts",
			Action:  actions.PrintOperatorStatus,
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}
