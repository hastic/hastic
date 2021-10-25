import { shallowMount } from '@vue/test-utils'
import Graph from '@/components/Graph.vue'

describe('HelloWorld.vue', () => {
  it('renders props.msg when passed', () => {
    const msg = 'new message'
    const wrapper = shallowMount(Graph, {
      props: { msg }
    })
    expect(wrapper.text()).toMatch(msg)
  })
})
