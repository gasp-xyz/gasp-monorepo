use alloy_sol_types::sol;
use eigen_types::BLSApkRegistry;

sol! {
    #[derive(Debug)]
    struct Deposit {
        RequestId requestId;
        address depositRecipient;
        address tokenAddress;
        uint256 amount;
        uint256 timeStamp;
    }

    #[derive(Debug)]
    struct L2UpdatesToRemove {
        RequestId requestId;
        uint256[] l2UpdatesToRemove;
        uint256 timeStamp;
    }
}