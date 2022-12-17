from string import Template

template_str = """\
impl IndexRangeU$width for IR<$start, $end> {
    fn start(self) -> u$width {
        $start
    }
    fn end(self) -> u$width {
        $end
    }
}"""
template = Template(template_str)

if __name__ == "__main__":
    for start in range(8):
        for end in range(8):
            if start <= end:
                print(template.substitute({"width": 8, "start": start, "end": end}))
    for start in range(32):
        for end in range(32):
            if start <= end:
                print(template.substitute({"width": 32, "start": start, "end": end}))
