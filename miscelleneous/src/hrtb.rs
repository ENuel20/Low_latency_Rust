pub fn process_item<I, F>(
    item : vec<I>,
    handler : F

    ) -> impl for<'a> Fn(&'a I) -> Box<future<output = bool > + 'static>
where
    I : Clone + Send + 'static,
    F : for<'b> fn(&'a [I]) -> Result<bool, &'b str> + Clone + Send + 'static,
{

}
