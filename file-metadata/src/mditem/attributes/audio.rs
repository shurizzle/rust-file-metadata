use file_metadata_mditem_macros::def_attribute;

use core_foundation::date::CFDate;
use core_foundation::string::CFString;
use core_foundation::array::CFArray;
use core_foundation::number::CFNumber;
use core_foundation::boolean::CFBoolean;

def_attribute!(AppleLoopDescriptors, CFArray<CFString>);
def_attribute!(AppleLoopsKeyFilterType, CFString);
def_attribute!(AppleLoopsLoopMode, CFString);
def_attribute!(AppleLoopsRootKey, CFString);
def_attribute!(AudioChannelCount, CFNumber);
def_attribute!(AudioEncodingApplication, CFString);
def_attribute!(AudioSampleRate, CFNumber);
def_attribute!(AudioTrackNumber, CFNumber);
def_attribute!(Composer, CFString);
def_attribute!(IsGeneralMIDISequence, CFBoolean);
def_attribute!(KeySignature, CFString);
def_attribute!(Lyricist, CFString);
def_attribute!(MusicalGenre, CFString);
def_attribute!(MusicalInstrumentCategory, CFString);
def_attribute!(MusicalInstrumentName, CFString);
def_attribute!(RecordingDate, CFDate);
def_attribute!(RecordingYear, CFNumber);
def_attribute!(Tempo, CFNumber);
def_attribute!(TimeSignature, CFString);
