package money

import (
	"testing"
)

func TestMoney(t *testing.T) {

	t.Run("Zero value Money construction works", func(t *testing.T) {
		cash := *(new(Money))
		assertCashValue(t, cash, 0)
	})

	t.Run("Normal Money construction works", func(t *testing.T) {
		cash := Money(251)
		assertCashValue(t, cash, 251)
	})

	// Placeholder: test constructing Money via string

	t.Run("Adding Money works", func(t *testing.T) {
		res := Money(251) + Money(533)
		assertCashValue(t, res, 784)
	})

	t.Run("Subtracting Money works", func(t *testing.T) {
		res := Money(533) - Money(251)
		assertCashValue(t, res, 282)
		res = Money(251) - Money(533)
		assertCashValue(t, res, -282)
	})

	t.Run("Add-assigning Money works", func(t *testing.T) {
		res := Money(251)
		res += Money(533)
		assertCashValue(t, res, 784)
	})

	t.Run("Subtract-Assigning Money works", func(t *testing.T) {
		res := Money(251)
		res -= Money(533)
		assertCashValue(t, res, -282)
	})

	t.Run("Multiplying Money by integer works", func(t *testing.T) {
		product := Money(251) * 99
		assertCashValue(t, product, 24849)
	})

	t.Run("Dividing Money by integer gives expected result", func(t *testing.T) {
		div := Money(252) / 2
		assertCashValue(t, div, 126)
	})

	t.Run("Dividing Money by integer gives truncates", func(t *testing.T) {
		div := Money(252) / 5
		assertCashValue(t, div, 50)
	})

	t.Run("Money equality works", func(t *testing.T) {
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

	t.Run("Money comparison works", func(t *testing.T) {
		const errorMsg = "Comparison for Money type is broken"
		stack1 := Money(99)
		stack2 := Money(100)
		if stack1 > stack2 {
			t.Errorf(errorMsg)
		}
		if stack2 < stack1 {
			t.Errorf(errorMsg)
		}
	})

	// Placeholder: test string formatting
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
