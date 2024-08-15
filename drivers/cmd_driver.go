package drivers

import . "github.com/RockLikeAmadeus/fundcast/app/money"

type CmdDriver struct{}

func (d CmdDriver) GetTotalBalance() (Money, error) {
	return Money(0), nil
}
