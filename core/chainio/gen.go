package chainio

//go:generate mockgen -destination=./mocks/avs_subscriber.go -package=mocks github.com/mangata-finance/eigen-layer-monorepo/core/chainio AvsSubscriberer
//go:generate mockgen -destination=./mocks/avs_writer.go -package=mocks github.com/mangata-finance/eigen-layer-monorepo/core/chainio AvsWriterer
//go:generate mockgen -destination=./mocks/avs_reader.go -package=mocks github.com/mangata-finance/eigen-layer-monorepo/core/chainio AvsReaderer
