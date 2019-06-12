use file_metadata_mditem_macros::def_attribute;

use core_foundation::string::CFString;
use core_foundation::array::CFArray;
use core_foundation::number::CFNumber;
use core_foundation::boolean::CFBoolean;

def_attribute!(AudioBitRate, CFNumber);
def_attribute!(Codecs, CFArray<CFString>);
def_attribute!(DeliveryType, CFString);
def_attribute!(MediaTypes, CFArray<CFString>);
def_attribute!(Streamable, CFBoolean);
def_attribute!(TotalBitRate, CFNumber);
def_attribute!(VideoBitRate, CFNumber);
def_attribute!(Director, CFString);
def_attribute!(Producer, CFString);
def_attribute!(Genre, CFString);
def_attribute!(Performers, CFArray<CFString>);
def_attribute!(OriginalFormat, CFString);
def_attribute!(OriginalSource, CFString);
