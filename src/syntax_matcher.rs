use iter_toolkit::session_iterator::SessionIterator;

pub trait SyntaxMatcher<I:Clone,O,It:Iterator<Item=I> + Sized>{
    fn match_syntax(&self,it: &mut SessionIterator<I,It>) -> Option<O>;
}

pub struct WordSyntaxMatcher<O:Clone>{
    word:String,
    val:O
}

impl<O:Clone> WordSyntaxMatcher<O> {
    pub fn new<W:ToString>(word: W, val: O) -> Self { Self { word:word.to_string(), val } }
}

impl<O:Clone,It:Iterator<Item=char>+Sized> SyntaxMatcher<char,O,It> for WordSyntaxMatcher<O>{
    fn match_syntax(&self,it: &mut SessionIterator<char,It>) -> Option<O> {
        it.new_session();
        for c in self.word.chars(){
            if let Some(mc) = it.next(){
                if mc == c {
                    continue;
                }
            }
            it.revert_session();
            return None;
        }
        it.commit_session();
        Some(self.val.clone())
    }
}

pub enum CharacterSet{
    UpperCase,
    LowerCase,
    Character(char),
    Set(Vec<Self>)
}

impl CharacterSet {
    pub fn match_char(&self,chr:&char)->bool{
        match(self){
            CharacterSet::UpperCase => {chr.is_uppercase()},
            CharacterSet::LowerCase => {chr.is_lowercase()},
            CharacterSet::Character(ch) => {*ch == *chr},
            CharacterSet::Set(cs) => {
                cs.iter().fold(true, |p,r|p&&r.match_char(chr))
            }
        }
    }

    pub fn capture_word<It:Iterator<Item=char>+Sized>(&self,it: &mut SessionIterator<char,It>)->String{
        let mut word=String::new();
        let mut last_ok = true;
        while last_ok {
            it.new_session();
            if let Some(ref chr) = it.next(){
                if self.match_char(chr) {
                    word.push(*chr);
                    it.commit_session();
                }else{
                    it.revert_session();
                }
            }else{
                last_ok = false;
                it.commit_session();
            }
        }
        return word;
    }

    pub fn extend(self,cs:Self)->Self{
        if let CharacterSet::Set(mut cs_list) = self{
            cs_list.push(cs);
            CharacterSet::Set(cs_list)
        }else{
            CharacterSet::Set(vec![self,cs])
        }
    }

    pub fn extend_char(self,chr:char)->Self{
        if let CharacterSet::Set(mut cs_list) = self{
            cs_list.push(Self::Character(chr));
            CharacterSet::Set(cs_list)
        }else{
            CharacterSet::Set(vec![self,Self::Character(chr)])
        }
    }

    pub fn extend_chars<It:Iterator<Item=char>>(self,chrs:It)->Self{
        if let CharacterSet::Set(mut cs_list) = self{
            for chr in chrs{
                cs_list.push(Self::Character(chr));
            }
            CharacterSet::Set(cs_list)
        }else{
            let mut list =vec![self];
            for chr in chrs{
                list.push(Self::Character(chr));
            }
            CharacterSet::Set(list)
        }
    }
}

pub struct CaptureSyntaxMatcher<O,F:Fn(&str)->Option<O> >{
    criteria:CharacterSet,
    initiator: F
}

impl<O,F:Fn(&str)->Option<O>> CaptureSyntaxMatcher<O,F> {
    pub fn new(criteria: CharacterSet, initiator: F) -> Self { Self { criteria, initiator } }
}

impl<O,It:Iterator<Item=char>+Sized,F:Fn(&str)->Option<O>> SyntaxMatcher<char,O,It> for CaptureSyntaxMatcher<O,F>{
    fn match_syntax(&self,it: &mut SessionIterator<char,It>) -> Option<O> {
        let word = self.criteria.capture_word(it);
        (self.initiator)(word.as_str())
    }
}

pub struct CompoundSyntaxMatcher<'a,O,Of,F:Fn(Vec<O>)->Option<Of>,I:Clone,It:Iterator<Item=I> + Sized>{
    parts:Vec<&'a dyn SyntaxMatcher<I,O,It>>,
    initiator:F
}

impl<'a,O,Of,F:Fn(Vec<O>)->Option<Of>,I:Clone,It:Iterator<Item=I> + Sized> CompoundSyntaxMatcher<'a, O, Of, F, I, It> {
    pub fn new(parts: Vec<&'a dyn SyntaxMatcher<I,O,It>>, initiator: F) -> Self { Self { parts, initiator } }
}

impl<'a,O,Of,F:Fn(Vec<O>)->Option<Of>,I:Clone,It:Iterator<Item=I> + Sized> SyntaxMatcher<I,Of,It> for CompoundSyntaxMatcher<'a, O, Of, F, I, It>{
    fn match_syntax(&self,it: &mut SessionIterator<I,It>) -> Option<Of> {
        it.new_session();
        let mut parsed_parts = Vec::new();
        for sm in self.parts.iter(){
            if let Some(parsed) = sm.match_syntax(it){
                parsed_parts.push(parsed);
            }else{
                it.revert_session();
                return None;
            }
        }
        it.commit_session();
        (self.initiator)(parsed_parts)
    }
}

pub struct OrSyntaxMatcher<'a,O,Of,F:Fn(O)->Option<Of>,I:Clone,It:Iterator<Item=I> + Sized>{
    parts:Vec<&'a dyn SyntaxMatcher<I,O,It>>,
    initiator:F
}

impl<'a,O,Of,F:Fn(O)->Option<Of>,I:Clone,It:Iterator<Item=I> + Sized> OrSyntaxMatcher<'a, O, Of, F, I, It> {
    pub fn new(parts: Vec<&'a dyn SyntaxMatcher<I,O,It>>, initiator: F) -> Self { Self { parts, initiator } }
}

impl<'a,O,Of,F:Fn(O)->Option<Of>,I:Clone,It:Iterator<Item=I> + Sized> SyntaxMatcher<I,Of,It> for OrSyntaxMatcher<'a, O, Of, F, I, It>{
    fn match_syntax(&self,it: &mut SessionIterator<I,It>) -> Option<Of> {
        let mut parsed_syntax=None;
        for sm in self.parts.iter(){
            it.new_session();
            if let Some(parsed) = sm.match_syntax(it){
                parsed_syntax=Some(parsed);
                it.commit_session();
                break
            }else{
                it.revert_session();
            }
        }
        if let Some(ps)=parsed_syntax{
            (self.initiator)(ps)
        }else{
            None
        }
    }
}



