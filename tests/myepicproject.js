const anchor = require('@project-serum/anchor');

const {SystemProgram} = anchor.web3;

const main = async () => {
    console.log('🚀 Starting test...')

    anchor.setProvider(anchor.Provider.env());
    const program = anchor.workspace.Myepicproject;

    const provider = anchor.Provider.env();
    const  baseAccount = anchor.web3.Keypair.generate();

    let tx = await program.rpc.startStuffOff({
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount],
    })

    console.log("Your Transaction signature:", tx);

    let account = await program.account.baseAccount.fetch(baseAccount.publicKey)
    console.log('👀 GIF Count', account.totalGifs.toString());

    await program.rpc.addGif("https://giphy.com/cool_gif_here", {
        accounts:{
            baseAccount: baseAccount.publicKey,
        },
    });

    account = await program.account.baseAccount.fetch(baseAccount.publicKey)
    console.log('👀 GIF Count', account.totalGifs.toString());

    console.log('👀 GIF List', account.gifList)

}

const runMain = async () => {
    try {
      await main();
      process.exit(0);
    } catch (error) {
      console.error(error);
      process.exit(1);
    }
  };
  
  runMain();