package specifications

import (
	"testing"

	. "github.com/RockLikeAmadeus/fundcast/app/money"
	"github.com/alecthomas/assert/v2"
)

type BalanceChecker interface {
	GetTotalBalance() (Money, error)
}

func Specification_CheckBalance(t testing.TB, balanceChecker BalanceChecker) {
	balance, err := balanceChecker.GetTotalBalance()
	assert.NoError(t, err)
	assert.Equal(t, balance, Money(10000))
}
