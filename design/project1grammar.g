grammar project1grammar;


program
    : HAI body KTHXBYE
    ;

body
    : element_list
    | 
    ;

element_list
    : element element_list
    |
    ;

element
    : comment
    | head_block
    | paragraph_block
    | bold_block
    | italics_block
    | list_block
    | newline
    | sound_block
    | video_block
    | var_def
    | var_use
    | text
    ;

comment
    : OBTW text TLDR
    ;

head_block
    : MAEK_HEAD head_content OIC
    ;

head_content
    : title_block
    | 
    ;

title_block
    : GIMMEH_TITLE text MKAY
    ;

paragraph_block
    : MAEK_PARAGRAF paragraph_element_list OIC
    ;

paragraph_element_list
    : paragraph_element paragraph_element_list
    | 
    ;

paragraph_element
    : bold_block
    | italics_block
    | list_block
    | sound_block
    | video_block
    | newline
    | var_def
    | var_use
    | text
    ;

bold_block
    : GIMMEH_BOLD text MKAY
    ;

italics_block
    : GIMMEH_ITALICS text MKAY
    ;

list_block
    : MAEK_LIST item_list OIC
    ;

item_list
    : list_item item_list
    | 
    ;

list_item
    : GIMMEH_ITEM item_content MKAY
    ;

item_content
    : bold_block
    | italics_block
    | text
    ;

newline
    : GIMMEH_NEWLINE
    ;

sound_block
    : GIMMEH_SOUNDZ address MKAY
    ;

video_block
    : GIMMEH_VIDZ address MKAY
    ;

var_def
    : I HAZ var_name IT IZ text MKAY
    ;

var_use
    : LEMME_SEE var_name MKAY
    ;

var_name
    : ID
    ;

address
    : TEXT
    ;

text
    : TEXT
    ;

HAI           : '#HAI' ;
KTHXBYE       : '#KTHXBYE' ;
OBTW          : '#OBTW' ;
TLDR          : '#TLDR' ;
MAEK_HEAD     : '#MAEK HEAD' ;
GIMMEH_TITLE  : '#GIMMEH TITLE' ;
MKAY          : '#MKAY' ;
OIC           : '#OIC' ;
MAEK_PARAGRAF : '#MAEK PARAGRAF' ;
GIMMEH_BOLD   : '#GIMMEH BOLD' ;
GIMMEH_ITALICS: '#GIMMEH ITALICS' ;
MAEK_LIST     : '#MAEK LIST' ;
GIMMEH_ITEM   : '#GIMMEH ITEM' ;
GIMMEH_NEWLINE: '#GIMMEH NEWLINE' ;
GIMMEH_SOUNDZ : '#GIMMEH SOUNDZ' ;
GIMMEH_VIDZ   : '#GIMMEH VIDZ' ;
I             : '#I' ;
HAZ           : 'HAZ' ;
IT            : '#IT' ;
IZ            : 'IZ' ;
LEMME_SEE     : '#LEMME SEE' ;


LETTER        : 'A'..'Z'| 'a'..'z' ;
DIGIT         : '0'..'9';
PUNCT         : ','|'"'|'.'|':'|'?'|'!'|'%'|'/';


ID            : LETTER (LETTER|DIGIT)* ;
TEXT          : (LETTER|DIGIT|PUNCT|' ')+ ;
