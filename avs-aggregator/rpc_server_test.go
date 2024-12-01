package aggregator

package aggregator

import (
	"context"
	"encoding/hex"
	"errors"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockLogger struct{}

func (m *MockLogger) Info(msg string, keysAndValues ...interface{}) {}
func (m *MockLogger) Error(msg string, keysAndValues ...interface{}) {}

type MockBlsAggregationService struct {
	mock.Mock
}

func (m *MockBlsAggregationService) ProcessNewSignature(ctx context.Context, taskId types.TaskId, digest sdktypes.TaskResponseDigest, signature *BlsSignature, operatorId [32]byte) error {
	args := m.Called(ctx, taskId, digest, signature, operatorId)
	return args.Error(0)
}

func TestProcessSignedTaskResponse(t *testing.T) {
	mockLogger := &MockLogger{}
	mockBlsAggregationService := &MockBlsAggregationService{}

	agg := &Aggregator{
		logger:                  mockLogger,
		blsAggregationService:   mockBlsAggregationService,
		taskResponses:           make(map[types.TaskId]map[sdktypes.TaskResponseDigest]interface{}),
		taskResponsesMu:         &sync.Mutex{},
	}
}
