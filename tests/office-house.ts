import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { OfficeHouse } from '../target/types/office_house';

describe('office-house', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.OfficeHouse as Program<OfficeHouse>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
