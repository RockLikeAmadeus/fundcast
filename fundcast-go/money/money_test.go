package money

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMoney(t *testing.T) {
	assert := assert.New(t)

	t.Run("Test zero value", func(t *testing.T) {
		cash := new(Money)
		assert.Equal(cash.value_as_int64(), 0)
	})

	// cash := new(Money)
	assert.Equal(1, 0)
	// assert.Equal(cash.value_as_int64(), 0)
	// assert.Equal(cash.major_part(), 0)
	// assert.Equal(cash.minor_part(), 0)
}
