package utils

import (
	"log"
	"math/rand"
	"time"
)

// Logger represents a logger
type Logger struct {
	*log.Logger
}

// NewLogger creates a new logger
func NewLogger() *Logger {
	return &Logger{
		Logger: log.New(log.Writer(), "ShardMaster: ", log.LstdFlags),
	}
}

// RandInt returns a random integer
func RandInt(min, max int) int {
	return rand.Intn(max-min) + min
}

// Sleep sleeps for a random duration
func Sleep(min, max time.Duration) {
	time.Sleep(time.Duration(RandInt(int(min), int(max))))
}
