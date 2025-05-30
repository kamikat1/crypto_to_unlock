use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgvNjqT9Efdz"); // substitua com o ID real do programa depois do deploy

#[program]
pub mod sale_program {
    use super::*;

    pub fn create_sale(
        ctx: Context<CreateSale>,
        sale_id: String,
        content_hash: String,
        preview_hash: String,
        amount: u64,
    ) -> Result<()> {
        let sale = &mut ctx.accounts.sale;
        sale.sale_id = sale_id;
        sale.buyer = *ctx.accounts.buyer.key;
        sale.seller = *ctx.accounts.seller.key;
        sale.content_hash = content_hash;
        sale.preview_hash = preview_hash;
        sale.amount = amount;
        sale.timestamp = Clock::get()?.unix_timestamp;

        let fee = amount / 20; // 5%
        let seller_amount = amount - fee;

        // Transfere a taxa para a plataforma
        **ctx.accounts.buyer.try_borrow_mut_lamports()? -= fee;
        **ctx.accounts.platform.try_borrow_mut_lamports()? += fee;

        // Transfere o restante para o vendedor
        **ctx.accounts.buyer.try_borrow_mut_lamports()? -= seller_amount;
        **ctx.accounts.seller.try_borrow_mut_lamports()? += seller_amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSale<'info> {
    #[account(init, payer = buyer, space = 8 + 256)]
    pub sale: Account<'info, Sale>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: Seller account is provided by the user and must be trusted.
    #[account(mut)]
    pub seller: AccountInfo<'info>,

    /// CHECK: Platform account is predefined and trusted.
    #[account(mut)]
    pub platform: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Sale {
    pub sale_id: String,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub content_hash: String,
    pub preview_hash: String,
    pub amount: u64,
    pub timestamp: i64,
}
