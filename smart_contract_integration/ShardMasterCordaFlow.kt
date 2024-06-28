package org.shardmaster.corda.flow

import co.paralleluniverse.fibers.Suspendable
import net.corda.core.contracts.CommandData
import net.corda.core.contracts.CommandWithParties
import net.corda.core.contracts.Contract
import net.corda.core.contracts.requireThat
import net.corda.core.flows.*
import net.corda.core.identity.Party
import net.corda.core.transactions.SignedTransaction
import net.corda.core.transactions.TransactionBuilder
import net.corda.core.utilities.ProgressTracker

class ShardMasterCordaFlow : FlowLogic<SignedTransaction>() {
    companion object {
        object GENERATING_TRANSACTION : ProgressTracker.Step("Generating transaction based on new shard.")
        object VERIFYING_TRANSACTION : ProgressTracker.Step("Verifying contract constraints.")
        object SIGNING_TRANSACTION : ProgressTracker.Step("Signing transaction with our private key.")
        object GATHERING_SIGS : ProgressTracker.Step("Gathering the counterparty'ssignature.") {
            override fun childProgressTracker() = ProgressTracker()
        }

        fun tracker() = ProgressTracker(GENERATING_TRANSACTION, VERIFYING_TRANSACTION, SIGNING_TRANSACTION, GATHERING_SIGS)
    }

    @Suspendable
    override fun call(): SignedTransaction {
        // Create a new shard
        val shard = Shard("newShard")

        // Create a transaction builder
        val txBuilder = TransactionBuilder(notary = serviceHub.networkMapCache.notaryIdentities[0])

        // Add the shard to the transaction
        txBuilder.addOutputState(shard, ShardContract.ID)

        // Add a command to the transaction
        txBuilder.addCommand(CommandWithParties(ShardContract.Commands.CreateShard(), listOf(ourIdentity)))

        // Verify the transaction
        txBuilder.verify(serviceHub)

        // Sign the transaction
        val signedTx = serviceHub.signInitialTransaction(txBuilder)

        // Gather the counterparty's signature
        val fullySignedTx = subFlow(CollectSignaturesFlow(signedTx, setOf(ourIdentity), GATHERING_SIGS))

        return fullySignedTx
    }
}
