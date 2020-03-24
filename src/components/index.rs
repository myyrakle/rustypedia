use yew::*;

pub struct Index
{}

pub enum Message
{}

impl Component for Index
{
    type Message = Message;
    type Properties = ();

    fn create(_:Self::Properties, link:ComponentLink<Self>) -> Self
    {
        Index{}
    }

    fn view(&self) -> Html 
    {
        html! 
        {
            <div>
            {"테스트"}
            
            </div>
        }
    }

    fn update(&mut self,  message:Self::Message) -> ShouldRender
    {
        match message
        {
            _ => false
        }
    }
}