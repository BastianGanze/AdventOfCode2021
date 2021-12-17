use crate::bit_reader::BitReaderBufferType;
use crate::ParseOutput;

#[derive(Debug)]
pub enum PackageType {
    Literal(BitReaderBufferType),
    Operator(BitReaderBufferType, Vec<Package>),
}

#[derive(Debug)]
pub struct Package {
    pub version: BitReaderBufferType,
    pub package_type: PackageType,
    package_length: BitReaderBufferType,
}

impl Package {
    pub fn new(
        version: BitReaderBufferType,
        package_type: PackageType,
        package_length: BitReaderBufferType,
    ) -> Package {
        Package {
            version,
            package_type,
            package_length,
        }
    }

    pub fn get_package_length(&self) -> BitReaderBufferType {
        return self.package_length.clone();
    }
}

pub fn read_package(bit_reader: &mut ParseOutput) -> Package {
    let (version, package_type, version_and_type_bits) = read_version_and_type(bit_reader);

    if package_type == 4 {
        let (package_value, package_value_bits) = read_literal_package_value(bit_reader);
        return Package::new(
            version,
            PackageType::Literal(package_value),
            version_and_type_bits + package_value_bits,
        );
    }

    let (sub_packages, bit_read_while_reading_sub_packages) =
        read_operator_sub_packages(bit_reader);

    return Package::new(
        version,
        PackageType::Operator(package_type, sub_packages),
        version_and_type_bits + bit_read_while_reading_sub_packages,
    );
}

pub fn read_version_and_type(
    bit_reader: &mut ParseOutput,
) -> (
    BitReaderBufferType,
    BitReaderBufferType,
    BitReaderBufferType,
) {
    (bit_reader.read_bits(3), bit_reader.read_bits(3), 6)
}

pub fn read_literal_package_value(
    bit_reader: &mut ParseOutput,
) -> (BitReaderBufferType, BitReaderBufferType) {
    let mut literal_value = 0;
    let mut bits_read = 0;

    loop {
        let is_end = bit_reader.read_bits(1) == 0;
        literal_value |= bit_reader.read_bits(4);

        bits_read += 5;
        if is_end {
            return (literal_value, bits_read);
        }
        literal_value = literal_value << 4;
    }
}

pub fn read_operator_sub_packages(
    bit_reader: &mut ParseOutput,
) -> (Vec<Package>, BitReaderBufferType) {
    let mut bits_read_while_parsing_sub_packages = 0;
    let length_type = bit_reader.read_bits(1);
    bits_read_while_parsing_sub_packages += 1;

    let mut sub_packages = Vec::new();

    if length_type == 0 {
        let mut bits_to_read = bit_reader.read_bits(15);
        bits_read_while_parsing_sub_packages += 15;

        loop {
            if bits_to_read == 0 {
                break;
            }
            let next_package = read_package(bit_reader);
            let package_length = next_package.get_package_length();
            bits_read_while_parsing_sub_packages += package_length;
            bits_to_read -= package_length;
            sub_packages.push(next_package);
        }
    } else {
        let packages_to_read = bit_reader.read_bits(11);
        bits_read_while_parsing_sub_packages += 11;

        for _ in 0..packages_to_read {
            let next_package = read_package(bit_reader);
            bits_read_while_parsing_sub_packages += next_package.get_package_length();
            sub_packages.push(next_package);
        }
    }

    (sub_packages, bits_read_while_parsing_sub_packages)
}
