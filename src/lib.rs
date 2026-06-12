use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CompactSize {
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BitcoinError {
    InsufficientBytes,
    InvalidFormat,
}

impl CompactSize {
    pub fn new(value: u64) -> Self {
        // TODO: Construct a CompactSize from a u64 value
         CompactSize {value}
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Encode according to Bitcoin's CompactSize format:
        // [0x00–0xFC] => 1 byte
        // [0xFDxxxx] => 0xFD + u16 (2 bytes)
        // [0xFExxxxxxxx] => 0xFE + u32 (4 bytes)
        // [0xFFxxxxxxxxxxxxxxxx] => 0xFF + u64 (8 bytes)
        let v = self.value;
        if v <= 0xFC {
            vec![v as u8]
        } else if v <= 0xFFFF {
            let mut out = vec![0xFD];
            out.extend_from_slice(&(v as u16).to_le_bytes());
            out
        } else if v <= 0xFFFFFFFF {
            let mut out = vec![0xFE];
            out.extend_from_slice(&(v as u32).to_le_bytes());
            out
        } else {
            let mut out = vec![0xFF];
            out.extend_from_slice(&v.to_le_bytes());
            out
        }



    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Decode CompactSize, returning value and number of bytes consumed.
        // First check if bytes is empty.
        // Check that enough bytes are available based on prefix.

        if bytes.is_empty() {
            return Err(BitcoinError::InsufficientBytes);
        }
 
        match bytes[0] {
            0xFD => {
                if bytes.len() < 3 {
                    return Err(BitcoinError::InsufficientBytes);
                }
                let val = u16::from_le_bytes([bytes[1], bytes[2]]);
                Ok((CompactSize::new(val as u64), 3))
            }
            0xFE => {
                if bytes.len() < 5 {
                    return Err(BitcoinError::InsufficientBytes);
                }
                let val = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                Ok((CompactSize::new(val as u64), 5))
            }
            0xFF => {
                if bytes.len() < 9 {
                    return Err(BitcoinError::InsufficientBytes);
                }
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                let val = u64::from_le_bytes(arr);
                Ok((CompactSize::new(val), 9))
            }
            n => Ok((CompactSize::new(n as u64), 1)),
        }
    
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Txid(pub [u8; 32]);

impl Serialize for Txid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Serialize as a hex-encoded string (32 bytes => 64 hex characters)
        let hex_string = hex::encode(self.0);
        serializer.serialize_str(&hex_string)
    }
}

impl<'de> Deserialize<'de> for Txid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: Parse hex string into 32-byte array
        // Use `hex::decode`, validate length = 32

        let s: String = Deserialize::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
 
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom(format!(
                "Invalid Txid length: expected 32 bytes, got {}",
                bytes.len()
            )));
        }
 
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Txid(arr))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: [u8; 32], vout: u32) -> Self {
        // TODO: Create an OutPoint from raw txid bytes and output index
        OutPoint {
            txid: Txid(txid),
            vout,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize as: txid (32 bytes) + vout (4 bytes, little-endian)
        let mut out = Vec::with_capacity(36);
        out.extend_from_slice(&self.txid.0);
        out.extend_from_slice(&self.vout.to_le_bytes());
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize 36 bytes: txid[0..32], vout[32..36]
        // Return error if insufficient bytes
        if bytes.len() < 36 {
            return Err(BitcoinError::InsufficientBytes);
        }
 
        let mut txid_arr = [0u8; 32];
        txid_arr.copy_from_slice(&bytes[0..32]);
 
        let vout = u32::from_le_bytes([bytes[32], bytes[33], bytes[34], bytes[35]]);
 
        Ok((OutPoint::new(txid_arr, vout), 36))
    
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn new(bytes: Vec<u8>) -> Self {
        // TODO: Simple constructor
        Script { bytes }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Prefix with CompactSize (length), then raw bytes
        let mut out = CompactSize::new(self.bytes.len() as u64).to_bytes();
        out.extend_from_slice(&self.bytes);
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Parse CompactSize prefix, then read that many bytes
        // Return error if not enough bytes
        let (compact, prefix_len) = CompactSize::from_bytes(bytes)?;
        let script_len = compact.value as usize;
 
        if bytes.len() < prefix_len + script_len {
            return Err(BitcoinError::InsufficientBytes);
        }
 
        let script_bytes = bytes[prefix_len..prefix_len + script_len].to_vec();
 
        Ok((Script::new(script_bytes), prefix_len + script_len))
    }
    }


impl Deref for Script {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        // TODO: Allow &Script to be used as &[u8]
        &self.bytes
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl TransactionInput {
    pub fn new(previous_output: OutPoint, script_sig: Script, sequence: u32) -> Self {
        // TODO: Basic constructor
        TransactionInput {
            previous_output,
            script_sig,
            sequence,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize: OutPoint + Script (with CompactSize) + sequence (4 bytes LE)
        let mut out = self.previous_output.to_bytes();
        out.extend_from_slice(&self.script_sig.to_bytes());
        out.extend_from_slice(&self.sequence.to_le_bytes());
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize in order:
        // - OutPoint (36 bytes)
        // - Script (with CompactSize)
        // - Sequence (4 bytes)

        let (previous_output, op_len) = OutPoint::from_bytes(bytes)?;
 
        let remaining = &bytes[op_len..];
        let (script_sig, script_len) = Script::from_bytes(remaining)?;
 
        let total_before_seq = op_len + script_len;
 
        if bytes.len() < total_before_seq + 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
 
        let seq_bytes = &bytes[total_before_seq..total_before_seq + 4];
        let sequence = u32::from_le_bytes([
            seq_bytes[0],
            seq_bytes[1],
            seq_bytes[2],
            seq_bytes[3],
        ]);
 
        Ok((
            TransactionInput::new(previous_output, script_sig, sequence),
            total_before_seq + 4,
        ))
    
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BitcoinTransaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub lock_time: u32,
}

impl BitcoinTransaction {
    pub fn new(version: u32, inputs: Vec<TransactionInput>, lock_time: u32) -> Self {
        // TODO: Construct a transaction from parts
        BitcoinTransaction {
            version,
            inputs,
            lock_time,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Format:
        // - version (4 bytes LE)
        // - CompactSize (number of inputs)
        // - each input serialized
        // - lock_time (4 bytes LE)

        let mut out = Vec::new();
 
        out.extend_from_slice(&self.version.to_le_bytes());
 
        out.extend_from_slice(&CompactSize::new(self.inputs.len() as u64).to_bytes());
 
        for input in &self.inputs {
            out.extend_from_slice(&input.to_bytes());
        }
 
        out.extend_from_slice(&self.lock_time.to_le_bytes());
 
        outs
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Read version, CompactSize for input count
        // Parse inputs one by one
        // Read final 4 bytes for lock_time
        if bytes.len() < 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
 
        let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let mut offset = 4;
 
        let (compact, compact_len) = CompactSize::from_bytes(&bytes[offset..])?;
        let input_count = compact.value as usize;
        offset += compact_len;
 
        let mut inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            let (input, input_len) = TransactionInput::from_bytes(&bytes[offset..])?;
            inputs.push(input);
            offset += input_len;
        }
 
        if bytes.len() < offset + 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
 
        let lock_time_bytes = &bytes[offset..offset + 4];
        let lock_time = u32::from_le_bytes([
            lock_time_bytes[0],
            lock_time_bytes[1],
            lock_time_bytes[2],
            lock_time_bytes[3],
        ]);
        offset += 4;
 
        Ok((BitcoinTransaction::new(version, inputs, lock_time), offset))
    
    }
}

impl fmt::Display for BitcoinTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format a user-friendly string showing version, inputs, lock_time
        // Display scriptSig length and bytes, and previous output info
    
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "Lock Time: {}", self.lock_time)?;
        writeln!(f, "Inputs: {}", self.inputs.len())?;
 
        for (i, input) in self.inputs.iter().enumerate() {
            writeln!(f, "  Input {}:", i)?;
            writeln!(
                f,
                "    Previous Output Txid: {}",
                hex::encode(input.previous_output.txid.0)
            )?;
            writeln!(
                f,
                "    Previous Output Vout: {}",
                input.previous_output.vout
            )?;
            writeln!(
                f,
                "    ScriptSig Length: {}",
                input.script_sig.bytes.len()
            )?;
            writeln!(
                f,
                "    ScriptSig Bytes: {}",
                hex::encode(&input.script_sig.bytes)
            )?;
            writeln!(f, "    Sequence: {}", input.sequence)?;
        }
 
        Ok(())
    
    }
}
