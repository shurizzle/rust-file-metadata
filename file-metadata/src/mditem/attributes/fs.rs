use file_metadata_mditem_macros::def_attribute;

use core_foundation::date::CFDate;
use core_foundation::string::CFString;
use core_foundation::number::CFNumber;
use core_foundation::boolean::CFBoolean;

def_attribute!(DisplayName, CFString);
def_attribute!(FSContentChangeDate, CFDate);
def_attribute!(FSCreationDate, CFDate);
def_attribute!(FSInvisible, CFBoolean);
def_attribute!(FSIsExtensionHidden, CFBoolean);
def_attribute!(FSLabel, CFNumber);
def_attribute!(FSName, CFString);
def_attribute!(FSNodeCount, CFNumber);
def_attribute!(FSOwnerGroupID, CFNumber);
def_attribute!(FSOwnerUserID, CFNumber);
def_attribute!(FSSize, CFNumber);
def_attribute!(Path, CFString);
