/**
 * * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

module.exports = {
  docs: [{
      type: 'doc',
      id: 'welcome',
    },
    {
      type: "category",
      label: 'Getting Started',
      link: {
          type: "doc",
          id:'getting_started/getting_started'

      },
      items: [
          {
              type: "doc",
              id: "getting_started/nodejs",
              label: 'Nodejs'
          },
          {
              type: "doc",
              id: "getting_started/python",
              label: 'Python'
          },
          {
              type: "doc",
              id: "getting_started/rust",
              label: 'Rust'
          },
      ]
    },
      {

          type: 'category',
          label: 'Explanations',
          items:[
              'explanations/library_overview',
              'explanations/account_approaches',
          ]
      },

    {

      type: 'category',
      label: 'How To',
      items:[
          {
              type: 'autogenerated',
              dirName: 'how_tos'
          },
      ]
    },
    {

      type: 'category',
      label: 'References',
      items:[
          {
              type: 'doc',
              id: 'references/rust_api_reference',
              label: 'Rust API Reference'
          },
          {
              type: 'doc',
              id: 'references/python_api_reference',
              label: 'Python API Reference'
          },
          {
              type: 'category',
              label: 'Node.js API Reference',
              link:{
                  type: "doc",
                  id: "references/nodejs/api_ref",
              },
              items:[
                  {
                      type: "category",
                      label: "Classes",
                      items:[
                          {
                              type: 'autogenerated',
                              dirName: 'references/nodejs/classes'
                          },
                      ]
                  },
                  {
                      type: "category",
                      label: "Enums",
                      items:[
                          {
                              type: 'autogenerated',
                              dirName: 'references/nodejs/enums'
                          },
                      ]
                  },
                  {
                      type: "category",
                      label: "Interfaces",
                      items:[
                          {
                              type: 'autogenerated',
                              dirName: 'references/nodejs/interfaces'
                          },
                      ]
                  },
              ]
          },
      ]
    },
    {
      type: 'doc',
      id: 'troubleshooting',
      label: 'Troubleshooting'
    },
    {
      type: 'doc',
      id: 'contribute',
      label: 'Contribute',
    }
  ]
};