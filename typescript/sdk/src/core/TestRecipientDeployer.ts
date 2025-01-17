import debug from 'debug';

import { TestRecipient, TestRecipient__factory } from '@hyperlane-xyz/core';
import { Address } from '@hyperlane-xyz/utils';

import { HyperlaneDeployer } from '../deploy/HyperlaneDeployer';
import { ContractVerifier } from '../deploy/verify/ContractVerifier';
import { MultiProvider } from '../providers/MultiProvider';
import { MailboxClientConfig } from '../router/types';
import { ChainName } from '../types';

export type TestRecipientConfig = Pick<
  MailboxClientConfig,
  'interchainSecurityModule'
>;

export type TestRecipientContracts = {
  testRecipient: TestRecipient;
};

export type TestRecipientAddresses = {
  testRecipient: Address;
};

export const testRecipientFactories = {
  testRecipient: new TestRecipient__factory(),
};

export class TestRecipientDeployer extends HyperlaneDeployer<
  TestRecipientConfig,
  typeof testRecipientFactories
> {
  constructor(
    multiProvider: MultiProvider,
    contractVerifier?: ContractVerifier,
  ) {
    super(multiProvider, testRecipientFactories, {
      logger: debug('hyperlane:TestRecipientDeployer'),
      contractVerifier,
    });
  }

  async deployContracts(
    chain: ChainName,
    config: TestRecipientConfig,
  ): Promise<TestRecipientContracts> {
    this.logger(`Deploying TestRecipient on ${chain}`, config);
    const testRecipient = await this.deployContract(chain, 'testRecipient', []);
    if (config.interchainSecurityModule) {
      this.logger(`Checking TestRecipient ISM on ${chain}`);
      await this.configureIsm(
        chain,
        testRecipient,
        config.interchainSecurityModule,
        (tr) => tr.interchainSecurityModule(),
        (tr, ism) => tr.populateTransaction.setInterchainSecurityModule(ism),
      );
    } else {
      this.logger(
        `WARNING: No ISM config provided for TestRecipient on ${chain}`,
      );
    }
    return {
      testRecipient,
    };
  }
}
