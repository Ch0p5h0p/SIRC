# The HyperCom Protocol

HyperCom aims to be secure. As such, there are some steps in encryption to maintain that security.

## Authentication
When a client registers a public key with a server for the first time, these steps happen:
1. Client "hello": Client sends its public key
2. Server acknowledgement: Server sends back its own public key
3. Server challenge: Server sends a challenge nonce
4. Client verification: client sends back Sign(nonce, client_private)
5. Server verification: server sends back both a nonce and Sign(nonce, server_private) for the cient to verify
If the server can't verify your keys, the attempt is rejected with error code 1.

## Servers
A server stores a few registries to keep everything in order:
### Users
The user registry is basically a dictionary, and maps public keys to nicks. On a HyperCom server, your identity is your public key. Alongside your nick, the server also maintains permissions and parameters related to you, such as operator status, timestamp of last sent message timestamp (UNIX epoch ms), etc.
### Blacklist
The blacklist registry includes both public keys and IPs that are prohibited from joining, and will be rejected upon authentication.
### Channels
The channels registry holds channel IDs, the channel label (such as "General"), channel subsribers, and private status. If a channel is private, subscribers will only be able to be added by the operator instead of by join request.

## Messages
Message sending is designed to be as secure as possible.
The client prepares the following data:
   - Message
   - Channel ID
   - private key
  
And does the following:
1. Encrypt the message with the server public key
2. Hash the message via SHA-256 and sign it with the private key

Then constructs this packet data:
```
Packet_data={
  channel_id,
  encrypted_message,
  signature
}
```
When the server receives this packet, it does the following:
1. Read the channel ID
2. Query subscribers
3. Decrypt message and encrypt it with each of the public keys from the subscribers, which should result in the packet data for each being:
```
Packet_data={
  channel_id,
  encrypted_message (subscriber_public),
  signature
}
```
effectively mirroring the original packet.

Then the message is sent to the client for decryption.

## Joining
Channel joining is pretty simple. You just have to have an authenticated client. The packet data is as follows:
```
Packet_data={
  signed_nonce,
  nonce,
  channel_id
}
```
The server verifies that the you're who you say you are, then attempts to add you to the channel subscriber list. If the channel is marked private, then it rejects the attempt with error code 2. If it fails to verify, you get error code 1.

## Leaving
Same as joining, basically.

## Broadcast Channel
The broadcast channel is a server channel, and is the only channel that maintains a history. Every user will get messages from the broadcast channel, and can't unsubscribe from it.

## Packet Structuring
Data in the packets is structured like so:
```
Packet={
  subprotocol_id,
  packet_data
}
```
Where the subprotocol id corresponds to this table:
| sub-protocol ID | sub-protocol name|
|---|---|
| 0 | authenticate |
| 1 | message |
| 2 | join |
| 3 | leave |

When a message is sent in binary, the data is in length-prefixed fields, where the length prefix is two bytes. Let's take the message, for example:
```
Packet_data={
  channel_id,
  encrypted_message,
  signature
}
```
In binary, this would be closer to `[channel_id_length][channel_id][encrypted_message_length][encrypted_essage][signature_length][signature]`
