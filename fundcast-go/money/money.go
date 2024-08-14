package money

type Money int64

func (m Money) MajorPart() int64 {
	return int64(m) / 100
}

func (m Money) MinorPart() int8 {
	return int8(int64(m) % 100)
}
