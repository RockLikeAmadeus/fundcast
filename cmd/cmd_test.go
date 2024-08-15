package cmd_test

import (
	"testing"

	"github.com/RockLikeAmadeus/fundcast/specifications"
)

func TestFundcastCommand(t *testing.T) {
	driver := something.Driver()
	specifications.Specification_CheckBalance(t, driver)
}
