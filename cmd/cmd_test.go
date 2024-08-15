package cmd_test

import (
	"testing"

	"github.com/RockLikeAmadeus/fundcast/drivers"
	"github.com/RockLikeAmadeus/fundcast/specifications"
)

func TestFundcastCommand(t *testing.T) {
	driver := drivers.CmdDriver{}
	specifications.Specification_CheckBalance(t, driver)
}
