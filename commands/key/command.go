package key

import "github.com/urfave/cli/v2"

func Command() *cli.Command {
	return &cli.Command{
		Name:        "key",
		Usage:       "key management utilities.",
		Subcommands: []*cli.Command{},
	}
}
