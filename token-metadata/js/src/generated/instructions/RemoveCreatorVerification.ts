/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';

/**
 * @category Instructions
 * @category RemoveCreatorVerification
 * @category generated
 */
const RemoveCreatorVerificationStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number;
}>([['instructionDiscriminator', beet.u8]], 'RemoveCreatorVerificationInstructionArgs');
/**
 * Accounts required by the _RemoveCreatorVerification_ instruction
 *
 * @property [_writable_] metadata Metadata (pda of ['metadata', program id, mint id])
 * @property [**signer**] creator Creator
 * @category Instructions
 * @category RemoveCreatorVerification
 * @category generated
 */
export type RemoveCreatorVerificationInstructionAccounts = {
  metadata: web3.PublicKey;
  creator: web3.PublicKey;
};

const removeCreatorVerificationInstructionDiscriminator = 28;

/**
 * Creates a _RemoveCreatorVerification_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category RemoveCreatorVerification
 * @category generated
 */
export function createRemoveCreatorVerificationInstruction(
  accounts: RemoveCreatorVerificationInstructionAccounts,
) {
  const { metadata, creator } = accounts;

  const [data] = RemoveCreatorVerificationStruct.serialize({
    instructionDiscriminator: removeCreatorVerificationInstructionDiscriminator,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: metadata,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: creator,
      isWritable: false,
      isSigner: true,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId: new web3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'),
    keys,
    data,
  });
  return ix;
}
