import sys
from nltk.corpus import wordnet as wn

print("# General symbols which we will use below")
print("$en = Ground(\"English Language\")")
print("$pos = Ground($en : \"Part of Speech\")")
print("$def = Ground($en : \"Definition\")")
print("$syn = Ground($en : \"Synonym\")")
print("$a = Ground($pos : \"Adjective\")")
print("$s = Ground($pos : \"Adjective Satellite\")")
print("$r = Ground($pos : \"Adverb\")")
print("$n = Ground($pos : \"Noun\")")
print("$v = Ground($pos : \"Verb\")")
print("# Link these definitions to words in the corpus, where appropriate")
print("# We can index into symbols we expect to be created. Cycles are not a problem")
print("Ground($a : $def_adverb.n.01)")
print("Ground($a : $def_adverb.n.01)")
print("$ss = Ground(\"Wordnet\" : \"Synset Name\")")
print("")
print("# Dump of synsets, lemmas, etc")


def main():
    if len(sys.argv) == 2:
        for synset in wn.synsets(sys.argv[1]):
            export_synset(synset)
    else:
        for synset in wn.all_synsets():
            export_synset(synset)


def export_synset(synset):
    ssn = "$ss_" + synset.name()

    # Wordnet organizes words into synsets, but this seems semantically wrong
    # Synonyms do NOT mean the same thing, but rather similar things
    # This is why relationships should probably not be hub and spoke
    # Connotations are a dimension of meaning

    print("\n# Synset: " + synset.name())

    dsym = "$def_" + synset.name().lower()
    print(dsym + " = Ground($def : \"" + synset.definition().lower() + "\")")

    lemma_symbols = []
    # , syn.pos(), ":",
    for l in synset.lemmas():  # Iterating through lemmas for each synset.

        lid = '$' + (synset.name() + "." + l.name()).lower()
        lemma_symbols.append(lid)
        print(lid + " = Ground(\"" + (l.name().lower()) + "\" : " + dsym + ")")

        # TODO - Record relationships between synonyms, antonyms, holonyms, etc

        #  l.frame_ids(), l.frame_strings())
        # l.syntactic_marker()

        # for a in l.antonyms():
        #     print("\tAntonym: ", a.name())

        # for d in l.derivationally_related_forms():
        #     print("\t\tDRF: ", d)
        # antonyms
        # hypernyms, instance_hypernyms
        # hyponyms, instance_hyponyms
        # member_holonyms, substance_holonyms, part_holonyms
        # member_meronyms, substance_meronyms, part_meronyms
        # topic_domains, region_domains, usage_domains
        # attributes
        # derivationally_related_forms
        # entailments
        # causes
        # also_sees
        # verb_groups
        # similar_tos
        # pertainyms

    print("\n# Record the synset name, and its association to the above symbols for posterity")
    print(ssn + " = Ground(DataNode($ss; \"" + synset.name() + "\"))")
    for ls in lemma_symbols:
        print(f"Ground({lid} : {ssn})")


main()
