import { OneClient } from '@superfaceai/one-sdk';

async function main() {
  const client = new OneClient({
    assetsPath: './superface',
    // token: <your-superface-token>
  });

  // Assign our CLI generated profile so it matches
  // the 'name' in the .profile file.
  // Example: email-communication/email-sending

  const profile = await client.getProfile('<profileName>');
  const useCase = profile.getUseCase('<usecaseName>'); // The <usecaseName> is also found in the .profile file

  try {
    const result = await useCase
      .perform(
        {
          // Input parameters as defined in profile:
          <key>: '<value>',
        },
        {
          provider: '<providerName>',
          parameters: {
            // Provider specific integration parameters:
            '<integrationParameterName>': '<integrationParameterValue>',
          },
          security: {
            // Provider specific security values:
            '<securityValueId>': {
              // Security values as described in provider or on profile page
            },
          },
        }
      );

  // output result on success
    console.log("RESULT:", JSON.stringify(result, null, 2));
  } catch(e) {
    // output result on error
    console.log("ERROR:", JSON.stringify(e, null, 2));
  }
}

main();
