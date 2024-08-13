package money

import (
	"testing"
)

func TestMoney(t *testing.T) {
	t.Run("Test Money equality", func(t *testing.T) {
		const errorMsg = "Equality for Money type is broken"
		stack1 := Money(99)
		stack2 := Money(99)
		if stack1 != stack2 {
			t.Errorf(errorMsg)
		}
		stack2 = Money(101)
		if stack1 == stack2 {
			t.Errorf(errorMsg)
		}
	})

	// t.Run("Test zero value", func(t *testing.T) {
	// 	cash := new(Money)
	// 	assertCashValue(t, cash, Money(0))
	// })

	// cash := new(Money)
	// assert.Equal(cash.value_as_int64(), 0)
	// assert.Equal(cash.major_part(), 0)
	// assert.Equal(cash.minor_part(), 0)
}

// func assertCashValue(t testing.TB, expected, received *Money) {
// 	t.Helper()
// 	if expected != received {
// 		t.Errorf("Expected value %d, received value %d")
// 	}
// }
