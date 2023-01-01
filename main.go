package main

import (
	"fmt"
	"os"

	"github.com/urfave/cli/v2"
)

func main() {
	app := &cli.App{
		Name:    "tss-cli",
		Version: "v0.1.0",
		Authors: []*cli.Author{
			{
				Name:  "ququzone",
				Email: "xueping.yang@gmail.com",
			},
		},
		HelpName:  "tss-cli",
		Usage:     "client for tss",
		UsageText: "tss-cli <SUBCOMMAND>",
		// Commands:  commands.Commonds(),
	}

	if err := app.Run(os.Args); err != nil {
		fmt.Fprintf(os.Stderr, "ERROR: %s\n", err.Error())
	}
}
