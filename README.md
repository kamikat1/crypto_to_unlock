# Crypto to Unlock 

Crypto to Unlock is a Telegram bot designed for digital photo sales, leveraging Web3 technologies like Solana and Anchor for secure payments. It facilitates a decentralized process for buyers and sellers, ensuring content protection and payment security. Key features include:

Buyer-Seller Interaction: Buyers start by chatting with the bot to specify the content they wish to purchase (e.g., personalized photos).

Seller Confirmation: The bot asks the seller to confirm their participation by providing their Telegram handle (@username). The seller then sets a price for the content.

Content Request: The bot asks the buyer for the content they wish to purchase, establishing a clear communication channel for both parties.

Initial Negotiation: The bot notifies the seller of the buyer’s interest and prompts them to set the price for the content.

Purchase Confirmation: The bot asks the buyer to confirm the price, sending a unique confirmation code (e.g., "Enter 9372 to confirm").

Private Key & Security: Once the purchase is confirmed, the bot sends a unique, secure private key (QR code) for the transaction, with instructions on safe storage.

Content Preview: The seller sends a photo that is encrypted and processed to create a blurred and protected preview, visible to the buyer for a few seconds. The bot stores hashes of the original content and preview for integrity.

Transaction Verification: The bot ensures that the buyer who confirmed the transaction is the same person who initiated it by validating the Telegram user_id. This prevents man-in-the-middle attacks and confirmation fraud.

Content Delivery: If the buyer approves the preview, the full, unprotected content is delivered. The bot ensures the content matches the previously stored hash to prevent tampering.

Web3 Payment Integration via Solana: Payments are processed automatically using Solana, with a transaction fee directed to the bot's wallet and the remainder to the seller's wallet.

Encrypted Content: Purchased content is encrypted using symmetric encryption (e.g., AES), with the decryption key securely sent to the buyer, allowing them to view the content.


All logs maintain minimal sensitive data, ensuring privacy and following best practices for security.

Offer Expiry: Each sale has an expiration time. After that, the transaction is automatically canceled, and all sensitive data (keys, previews) is discarded.
