cmake_minimum_required(VERSION 3.22)
project(KnobMuse VERSION 0.1.0)

add_subdirectory(JUCE) # JUCE 7.0.12 as submodule

juce_add_plugin(KnobMuse
    COMPANY_NAME "tayo6"
    PLUGIN_MANUFACTURER_CODE Tay6
    PLUGIN_CODE Kmse
    FORMATS VST3 Standalone
    ProductName "KnobMuse"
    VST3_CATEGORIES Tools
)

juce_generate_juce_header(KnobMuse)

target_sources(KnobMuse PRIVATE
    Source/PluginProcessor.cpp
    Source/PluginEditor.cpp
)

target_link_libraries(KnobMuse PRIVATE
    juce::juce_audio_utils
    juce::juce_dsp
    juce::juce_gui_basics
)

juce_add_binary_data(KnobMuseData SOURCES "")