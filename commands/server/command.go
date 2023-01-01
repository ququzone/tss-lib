package server

import "github.com/urfave/cli/v2"

func Command() *cli.Command {
	return &cli.Command{
		Name:        "server",
		Usage:       "server management utilities.",
		Subcommands: []*cli.Command{},
	}
}
