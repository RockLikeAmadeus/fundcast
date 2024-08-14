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

	t.Run("Test zero value construction", func(t *testing.T) {
		cash := *(new(Money))
		assertCashValue(t, cash, 0)
		assertCashValue(t, Money(12345), 12345)
	})

	// cash := new(Money)
	// assert.Equal(cash.value_as_int64(), 0)
	// assert.Equal(cash.major_part(), 0)
	// assert.Equal(cash.minor_part(), 0)
}

func assertCashValue(t testing.TB, received Money, expected int64) {
	t.Helper()
	expMajorPart := expected / 100
	expMinorPart := int8(expected % 100)
	recMajorPart := received.MajorPart()
	recMinorPart := received.MinorPart()
	if expected != int64(received) {
		t.Errorf(
			"Expected total money value %d, received value %d",
			expected,
			received,
		)
	}
	if expMajorPart != recMajorPart {
		t.Errorf(
			"Expected major part of Money value %d, received value %d",
			expMajorPart,
			recMajorPart,
		)
	}
	if expMinorPart != recMinorPart {
		t.Errorf(
			"Expected minor part of Money value %d, received value %d",
			expMinorPart,
			recMinorPart,
		)
	}
}
