from string import Template

template_str = """\
impl IndexU$width for I<$value> {
    fn value(self) -> usize {
        $value
    }
}"""
template = Template(template_str)

if __name__ == "__main__":
    for i in range(8):
        print(template.substitute({"width": 8, "value": i}))
    for i in range(32):
        print(template.substitute({"width": 32, "value": i}))
