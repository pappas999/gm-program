import {
    establishConnection,
    establishPayer,
    checkProgram,
    sayGm,
    reportGm,
  } from './gm_program';

  async function main() {
    console.log("Let's say GM anon...");

    // Establish connection to the cluster
    await establishConnection();

    // Determine who pays for the fees
    await establishPayer();

    // Check if the program has been deployed
    await checkProgram();

    // Say hello to an account
    await sayGm();

    // Find out how many times that account has been greeted
    await reportGm();

    console.log('Success');
  }

  main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
  );