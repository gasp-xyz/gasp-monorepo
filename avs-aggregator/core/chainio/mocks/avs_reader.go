// Code generated by MockGen. DO NOT EDIT.
// Source: github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core/chainio (interfaces: AvsReaderer)
//
// Generated by this command:
//
//	mockgen -destination=./mocks/avs_reader.go -package=mocks github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core/chainio AvsReaderer
//
// Package mocks is a generated GoMock package.
package mocks

import (
	context "context"
	reflect "reflect"

	contractMangataTaskManager "github.com/gasp-xyz/gasp-monorepo/avs-aggregator/bindings/MangataTaskManager"
	gomock "go.uber.org/mock/gomock"
)

// MockAvsReaderer is a mock of AvsReaderer interface.
type MockAvsReaderer struct {
	ctrl     *gomock.Controller
	recorder *MockAvsReadererMockRecorder
}

// MockAvsReadererMockRecorder is the mock recorder for MockAvsReaderer.
type MockAvsReadererMockRecorder struct {
	mock *MockAvsReaderer
}

// NewMockAvsReaderer creates a new mock instance.
func NewMockAvsReaderer(ctrl *gomock.Controller) *MockAvsReaderer {
	mock := &MockAvsReaderer{ctrl: ctrl}
	mock.recorder = &MockAvsReadererMockRecorder{mock}
	return mock
}

// EXPECT returns an object that allows the caller to indicate expected use.
func (m *MockAvsReaderer) EXPECT() *MockAvsReadererMockRecorder {
	return m.recorder
}

// CheckSignatures mocks base method.
func (m *MockAvsReaderer) CheckSignatures(arg0 context.Context, arg1 [32]byte, arg2 []byte, arg3 uint32, arg4 contractMangataTaskManager.IBLSSignatureCheckerNonSignerStakesAndSignature) (contractMangataTaskManager.IBLSSignatureCheckerQuorumStakeTotals, error) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "CheckSignatures", arg0, arg1, arg2, arg3, arg4)
	ret0, _ := ret[0].(contractMangataTaskManager.IBLSSignatureCheckerQuorumStakeTotals)
	ret1, _ := ret[1].(error)
	return ret0, ret1
}

// CheckSignatures indicates an expected call of CheckSignatures.
func (mr *MockAvsReadererMockRecorder) CheckSignatures(arg0, arg1, arg2, arg3, arg4 any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "CheckSignatures", reflect.TypeOf((*MockAvsReaderer)(nil).CheckSignatures), arg0, arg1, arg2, arg3, arg4)
}
